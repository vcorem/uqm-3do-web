import { describe, expect, it } from 'vitest';
import * as THREE from 'three';
import { assetURL, installAssetBase } from './base';

describe('files under the site path /cosmoclash/', () => {
  it('puts the base in front of root and relative paths once', () => {
    expect(assetURL('/textures/logo.jpg', '/cosmoclash/')).toBe('/cosmoclash/textures/logo.jpg');
    expect(assetURL('textures/logo.jpg', '/cosmoclash/')).toBe('/cosmoclash/textures/logo.jpg');
    expect(assetURL('/cosmoclash/textures/logo.jpg', '/cosmoclash/')).toBe('/cosmoclash/textures/logo.jpg');
    expect(assetURL('cosmoclash.wasm', '/cosmoclash')).toBe('/cosmoclash/cosmoclash.wasm');
    expect(assetURL('/models/gondi.glb', '/')).toBe('/models/gondi.glb');
  });
  it('keeps full, data and blob URLs', () => {
    for (const url of ['data:image/png;base64,AA', 'blob:http://x/1', 'https://cdn.example/a.png', '//cdn.example/a.png']) {
      expect(assetURL(url, '/cosmoclash/')).toBe(url);
    }
  });
  it('the loaders of the upstream scene code get the base', () => {
    const manager = new THREE.LoadingManager();
    installAssetBase(manager, '/cosmoclash/');
    expect(manager.resolveURL('/models/gondi.glb')).toBe('/cosmoclash/models/gondi.glb');
    expect(manager.resolveURL('/textures/star_test-02.jpg')).toBe('/cosmoclash/textures/star_test-02.jpg');
    expect(manager.resolveURL('blob:http://x/1')).toBe('blob:http://x/1');
    expect(manager.resolveURL('/cosmoclash/models/gondi.glb')).toBe('/cosmoclash/models/gondi.glb');
  });
});
