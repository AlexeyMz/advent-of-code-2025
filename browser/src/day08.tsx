import * as THREE from 'three';

import {
  StageWithGrid, checkGraphicsSupport, makeColorForIndex,
} from './stageWithGrid';

import DATA from '@/data/output/puzzle08_data.json';

class MainStage extends StageWithGrid {
  private bricks: Bricks;

  constructor() {
    super({
      container: document.body,
      cameraNear: 0.1,
      cameraFar: 1000,
    });
    this.bricks = new Bricks(this.scene);
    this.controls.target = this.bricks.centerAtGround;
  }

  override update(): void {
    super.update();
    this.bricks.update();
  }
}

class Bricks {
  readonly centerAtGround: THREE.Vector3;

  constructor(scene: THREE.Scene) {
    let minX = 0;
    let minY = 0;
    let maxX = 0;
    let maxY = 0;
    for (let i = 0; i < DATA.boxes.length; i++) {
      const [x, y, z] = DATA.boxes[i].map(c => c * DATA.scale);
      const geometry = new THREE.IcosahedronGeometry(0.1);
      const color = makeColorForIndex(i);
      const material = new THREE.MeshPhysicalMaterial({color});
      const mesh = new THREE.Mesh(geometry, material);
      mesh.position.set(x, y, z);
      scene.add(mesh);
      minX = Math.min(minX, x);
      minY = Math.min(minY, y);
      maxX = Math.max(maxX, x);
      maxY = Math.max(maxY, y);
    }
    this.centerAtGround = new THREE.Vector3(
      (maxX - minX) / 2,
      1,
      (maxY - minY) / 2
    );
  }

  update() {
    // this.cube.rotation.x += 0.01;
    // this.cube.rotation.y += 0.01;
  }
}

const support = checkGraphicsSupport();
if (support.error) {
  document.body.appendChild(support.error);
} else {
  const mainStage = new MainStage();
  mainStage.run();
}
