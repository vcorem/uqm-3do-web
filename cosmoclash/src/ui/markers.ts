import * as THREE from 'three';

export const PLAYER_COLORS = ['#3fd6ff', '#ffa53f'] as const;

function canvasTexture(size: number, draw: (ctx: CanvasRenderingContext2D) => void): THREE.CanvasTexture {
  const canvas = document.createElement('canvas');
  canvas.width = canvas.height = size;
  const ctx = canvas.getContext('2d');
  if (ctx) draw(ctx);
  const texture = new THREE.CanvasTexture(canvas);
  texture.colorSpace = THREE.SRGBColorSpace;
  return texture;
}
function sprite(texture: THREE.Texture, scale: number): THREE.Sprite {
  const material = new THREE.SpriteMaterial({ map: texture, transparent: true, depthTest: false, depthWrite: false });
  const node = new THREE.Sprite(material);
  node.scale.set(scale, scale, 1);
  node.renderOrder = 10;
  return node;
}
/** A glowing ring in a player's colour, around a ship at the start of a battle. */
export function ringSprite(color: string, scale = 90): THREE.Sprite {
  return sprite(canvasTexture(128, ctx => {
    ctx.strokeStyle = color;
    ctx.shadowColor = color;
    ctx.shadowBlur = 12;
    ctx.lineWidth = 7;
    ctx.beginPath(); ctx.arc(64, 64, 50, 0, Math.PI * 2); ctx.stroke();
  }), scale);
}
/** A short label ("YOU", "P2") in a player's colour. */
export function labelSprite(text: string, color: string, scale = 60): THREE.Sprite {
  return sprite(canvasTexture(256, ctx => {
    ctx.font = 'bold 72px "Segoe UI", Tahoma, Arial, sans-serif';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.shadowColor = '#000';
    ctx.shadowBlur = 10;
    ctx.fillStyle = color;
    ctx.fillText(text, 128, 128);
  }), scale);
}
/** A round badge on a ship of the fleet: 1 on the ship that opens the match, a check mark on the others. */
export function badgeSprite(n: number | string, color: string, scale = 22): THREE.Sprite {
  return sprite(canvasTexture(128, ctx => {
    ctx.fillStyle = color;
    ctx.shadowColor = color;
    ctx.shadowBlur = 16;
    ctx.beginPath(); ctx.arc(64, 64, 46, 0, Math.PI * 2); ctx.fill();
    ctx.shadowBlur = 0;
    ctx.fillStyle = '#001018';
    ctx.font = 'bold 64px "Segoe UI", Tahoma, Arial, sans-serif';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(String(n), 64, 68);
  }), scale);
}
export function disposeSprite(node: THREE.Sprite): void {
  node.material.map?.dispose();
  node.material.dispose();
  node.removeFromParent();
}
