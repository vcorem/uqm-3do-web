import * as THREE from 'three';

/** The site path of the game: '/cosmoclash/' in a build, '/' in unit tests. */
export const BASE: string = import.meta.env.BASE_URL || '/';

/**
 * The URL of a file in public/ under the site path. The original code names
 * its files from the site root ('/textures/logo.jpg'); under /cosmoclash/ that
 * becomes '/cosmoclash/textures/logo.jpg'. Full URLs (data:, blob:, http:) and
 * paths that already start with the base do not change.
 */
export function assetURL(path: string, base: string = BASE): string {
  if (/^(?:[a-z][a-z0-9+.-]*:|\/\/)/i.test(path)) return path;
  const root = base.endsWith('/') ? base : base + '/';
  if (path.startsWith(root)) return path;
  return root + path.replace(/^\/+/, '');
}

/**
 * The upstream scene code (src/game, kept as in the original game) loads its
 * models and textures with root paths through Three.js loaders. The default
 * loading manager puts the base in front of them.
 */
export function installAssetBase(manager: THREE.LoadingManager = THREE.DefaultLoadingManager, base: string = BASE): void {
  manager.setURLModifier(url => (url.startsWith('/') && !url.startsWith('//') ? assetURL(url, base) : url));
}
