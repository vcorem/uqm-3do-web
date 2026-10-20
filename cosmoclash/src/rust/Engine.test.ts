import {describe,it,expect} from 'vitest';
import {inputMask,maskInput,snapshotHash,type Snapshot} from './Engine';
describe('Rust browser host boundaries',()=>{
 it('preserves every network input combination',()=>{for(let bits=0;bits<32;bits++)expect(inputMask(maskInput(bits))).toBe(bits);});
 it('includes fleet transitions even when the engine also supplies a checksum',()=>{
  const a={tick:30,checksum:'same-engine-state',match:{indices:[0,0],battle:0}};
  const b={...a,match:{indices:[0,1],battle:1}};
  expect(snapshotHash(a as unknown as Snapshot)).not.toBe(snapshotHash(b as unknown as Snapshot));
  expect(snapshotHash(a as unknown as Snapshot)).toBe(snapshotHash(structuredClone(a) as unknown as Snapshot));
 });
});
