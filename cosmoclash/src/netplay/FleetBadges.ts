import * as THREE from 'three';
import { badgeSprite, disposeSprite } from '@/ui/markers';

/**
 * Number badges (1, 2, 3) over the ships of the 3D ship choice that are in
 * the player's fleet. The ship containers are the groups of the selection
 * scene, in the order of the character registry (src/game/SelectionScene.ts).
 */
export class FleetBadges {
  private badges: THREE.Sprite[] = [];
  private key = '';
  constructor(private readonly scene: THREE.Scene) {}
  set(picks: number[], color: string): void {
    const key = picks.join(',') + color;
    if (key === this.key) return;
    this.key = key;
    this.clear();
    const ships = this.scene.children.filter(child => child instanceof THREE.Group);
    picks.forEach((ship, n) => {
      const group = ships[ship];
      if (!group) return;
      const badge = badgeSprite(n + 1, color);
      // Left of the ship: the ship card is above it and the fleet tray below.
      badge.position.set(-44, 14, 0);
      group.add(badge);
      this.badges.push(badge);
    });
  }
  clear(): void { for (const badge of this.badges) disposeSprite(badge); this.badges = []; }
  dispose(): void { this.clear(); this.key = ''; }
}
