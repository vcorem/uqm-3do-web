import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { SoundManager } from './SoundManager';
import { SoundEffect, SOUND_CONFIG } from './SoundEffect';

const totalSounds = Object.keys(SOUND_CONFIG).length + 2;
const mockAudioContext = {
  createGain: vi.fn(() => ({ gain: { value: 1 }, connect: vi.fn() })),
  createBufferSource: vi.fn(() => ({
    buffer: null as AudioBuffer | null, loop: false,
    connect: vi.fn(), start: vi.fn(), stop: vi.fn(),
  })),
  decodeAudioData: vi.fn(async (buf: ArrayBuffer) => buf as unknown as AudioBuffer),
  destination: {}, state: 'suspended', resume: vi.fn(async () => {}),
};
const audioContextConstructor = vi.fn(function () { return mockAudioContext; });
const mockFetch = vi.fn(async () => ({ arrayBuffer: async () => new ArrayBuffer(8) }));

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>(done => { resolve = done; });
  return { promise, resolve };
}

// Loading is deliberately allowed to outlive the initial play call. Await this
// lifecycle in tests so prior singleton instances never leak work into a case.
async function decoded(sm: SoundManager) {
  await (sm as unknown as { decodePromise: Promise<void> | null }).decodePromise;
}

describe('SoundManager', () => {
  beforeEach(() => {
    SoundManager.reset();
    localStorage.clear();
    vi.clearAllMocks();
    mockAudioContext.decodeAudioData.mockImplementation(async buf => buf as unknown as AudioBuffer);
    mockFetch.mockImplementation(async () => ({ arrayBuffer: async () => new ArrayBuffer(8) }));
    vi.stubGlobal('AudioContext', audioContextConstructor);
    vi.stubGlobal('fetch', mockFetch);
    // Capture this instance's gesture callback without retaining listeners from
    // earlier reset singletons on the shared jsdom document.
    vi.spyOn(document, 'addEventListener').mockImplementation(() => {});
    vi.spyOn(document, 'removeEventListener').mockImplementation(() => {});
  });
  afterEach(() => { vi.restoreAllMocks(); });

  it('is a singleton', () => {
    expect(SoundManager.getInstance()).toBe(SoundManager.getInstance());
  });

  it('preload fetches everything once without creating an AudioContext', async () => {
    const sm = SoundManager.getInstance();
    const first = sm.preload();
    expect(sm.preload()).toBe(first);
    await expect(first).resolves.toBeUndefined();
    expect(mockFetch).toHaveBeenCalledTimes(totalSounds);
    expect(audioContextConstructor).not.toHaveBeenCalled();
  });

  it('setMasterVolume clamps to 0-1', () => {
    const sm = SoundManager.getInstance();
    sm.setMasterVolume(0.5);
    expect(sm.getMasterVolume()).toBe(0.5);
    sm.setMasterVolume(-1);
    expect(sm.getMasterVolume()).toBe(0);
    sm.setMasterVolume(2);
    expect(sm.getMasterVolume()).toBe(1);
  });

  it('setMuted toggles mute state', () => {
    const sm = SoundManager.getInstance();
    expect(sm.isMuted()).toBe(false);
    sm.setMuted(true);
    expect(sm.isMuted()).toBe(true);
    sm.setMuted(false);
    expect(sm.isMuted()).toBe(false);
  });

  it('play starts preload safely when called before preload', async () => {
    const sm = SoundManager.getInstance();
    expect(() => sm.play(SoundEffect.Hit)).not.toThrow();
    expect(audioContextConstructor).toHaveBeenCalledTimes(1);
    await decoded(sm);
    expect(mockAudioContext.decodeAudioData).toHaveBeenCalledTimes(totalSounds);
    sm.play(SoundEffect.Hit);
    expect(mockAudioContext.createBufferSource).toHaveBeenCalledTimes(1);
    expect(mockAudioContext.createBufferSource.mock.results[0].value.start).toHaveBeenCalledOnce();
  });

  it('persists volume and mute and restores them in the next instance', () => {
    const sm = SoundManager.getInstance();
    sm.setMasterVolume(0.3);
    sm.setMuted(true);
    expect(localStorage.getItem('cosmoclash-volume')).toBe('0.3');
    expect(localStorage.getItem('cosmoclash-muted')).toBe('true');
    SoundManager.reset();
    expect(SoundManager.getInstance().getMasterVolume()).toBe(0.3);
    expect(SoundManager.getInstance().isMuted()).toBe(true);
  });

  it('unlocks synchronously but waits for a slow fetch before decoding every sound', async () => {
    const slow = deferred<ArrayBuffer>();
    mockFetch.mockImplementationOnce(async () => ({ arrayBuffer: () => slow.promise }));
    const sm = SoundManager.getInstance();
    const preload = sm.preload();
    const unlock = vi.mocked(document.addEventListener).mock.calls.find(([type]) => type === 'click')![1] as EventListener;
    unlock(new Event('click'));
    // The context/resume operation must happen before the network await.
    expect(audioContextConstructor).toHaveBeenCalledOnce();
    expect(mockAudioContext.resume).toHaveBeenCalled();
    expect(document.removeEventListener).toHaveBeenCalledWith('click', unlock);
    sm.playMusic('intro');
    sm.play(SoundEffect.Hit);
    await Promise.resolve();
    expect(mockAudioContext.decodeAudioData).not.toHaveBeenCalled();
    slow.resolve(new ArrayBuffer(16));
    await preload;
    await decoded(sm);
    expect(mockAudioContext.decodeAudioData).toHaveBeenCalledTimes(totalSounds);
    expect(mockAudioContext.createBufferSource).toHaveBeenCalledOnce();
    const music = mockAudioContext.createBufferSource.mock.results[0].value;
    expect(music.loop).toBe(true);
    expect(music.start).toHaveBeenCalledOnce();
    sm.play(SoundEffect.PlasmaShot); // The late first fetch was not discarded.
    expect(mockAudioContext.createBufferSource).toHaveBeenCalledTimes(2);
    expect(mockAudioContext.decodeAudioData).toHaveBeenCalledTimes(totalSounds);
  });

  it('shares in-flight decoding and plays only the latest requested music', async () => {
    const buffers = new Map<string, ArrayBuffer>();
    mockFetch.mockImplementation(async (...args: unknown[]) => {
      const buffer = new ArrayBuffer(8);
      buffers.set(String(args[0]), buffer);
      return { arrayBuffer: async () => buffer };
    });
    const sm = SoundManager.getInstance();
    await sm.preload();
    const decoding = deferred<void>();
    mockAudioContext.decodeAudioData.mockImplementation(async buf => {
      await decoding.promise;
      return buf as unknown as AudioBuffer;
    });
    sm.playMusic('intro');
    await Promise.resolve();
    sm.playMusic('battle');
    sm.play(SoundEffect.Hit);
    expect(mockAudioContext.decodeAudioData).toHaveBeenCalledTimes(totalSounds);
    expect(mockAudioContext.createBufferSource).not.toHaveBeenCalled();
    decoding.resolve();
    await decoded(sm);
    expect(mockAudioContext.createBufferSource).toHaveBeenCalledOnce();
    expect(mockAudioContext.createBufferSource.mock.results[0].value.buffer).toBe(buffers.get('/sounds/MeleeTheme.ogg'));
    sm.playMusic('battle');
    expect(mockAudioContext.createBufferSource).toHaveBeenCalledOnce();
  });

  it('stopMusic cancels a pending request while fetches are still loading', async () => {
    const slow = deferred<ArrayBuffer>();
    mockFetch.mockImplementationOnce(async () => ({ arrayBuffer: () => slow.promise }));
    const sm = SoundManager.getInstance();
    sm.playMusic('intro');
    sm.stopMusic();
    slow.resolve(new ArrayBuffer(8));
    await decoded(sm);
    expect(mockAudioContext.createBufferSource).not.toHaveBeenCalled();
    sm.playMusic('battle');
    expect(mockAudioContext.createBufferSource).toHaveBeenCalledOnce();
    sm.stopMusic();
    expect(mockAudioContext.createBufferSource.mock.results[0].value.stop).toHaveBeenCalledOnce();
  });

  it('preserves master mute, persisted volume and per-effect gain after async decode', async () => {
    const sm = SoundManager.getInstance();
    sm.setMasterVolume(0.4);
    sm.setMuted(true);
    sm.playMusic('intro');
    await decoded(sm);
    expect(mockAudioContext.createGain.mock.results[0].value.gain.value).toBe(0);
    sm.play(SoundEffect.Hit);
    expect(mockAudioContext.createBufferSource).toHaveBeenCalledOnce(); // Music only, muted by master gain.
    sm.setMuted(false);
    expect(mockAudioContext.createGain.mock.results[0].value.gain.value).toBe(0.4);
    sm.play(SoundEffect.Hit);
    expect(mockAudioContext.createGain.mock.results[mockAudioContext.createGain.mock.results.length - 1].value.gain.value).toBe(SOUND_CONFIG[SoundEffect.Hit].volume);
    expect(mockAudioContext.createBufferSource).toHaveBeenCalledTimes(2);
  });
});
