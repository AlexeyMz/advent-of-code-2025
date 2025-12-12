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
    const scaledBoxes = DATA.boxes.map(b => b.map(c => c * DATA.scale));

    let minX = 0;
    let minY = 0;
    let maxX = 0;
    let maxY = 0;
    for (let i = 0; i < DATA.boxes.length; i++) {
      const [x, y, z] = scaledBoxes[i];
      const geometry = new THREE.IcosahedronGeometry(0.05);
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

    const linePositions = new Float32Array(DATA.edges.length * 6);
    const lineColors = new Float32Array(DATA.edges.length * 6);
    for (let i = 0; i < DATA.edges.length; i++) {
      const [fromIndex, toIndex] = DATA.edges[i];
      const from = scaledBoxes[fromIndex];
      const to = scaledBoxes[toIndex];

      const lineOffset = i * 6;
      linePositions[lineOffset + 0] = from[0];
      linePositions[lineOffset + 1] = from[1];
      linePositions[lineOffset + 2] = from[2];
      linePositions[lineOffset + 3] = to[0];
      linePositions[lineOffset + 4] = to[1];
      linePositions[lineOffset + 5] = to[2];

      const fromColor = new THREE.Color(makeColorForIndex(fromIndex));
      const toColor = new THREE.Color(makeColorForIndex(toIndex));
      lineColors[lineOffset + 0] = fromColor.r;
      lineColors[lineOffset + 1] = fromColor.g;
      lineColors[lineOffset + 2] = fromColor.b;
      lineColors[lineOffset + 3] = toColor.r;
      lineColors[lineOffset + 4] = toColor.g;
      lineColors[lineOffset + 5] = toColor.b;
    }

    const lineGeometry = new THREE.BufferGeometry();
    lineGeometry.setAttribute('position', new THREE.BufferAttribute(linePositions, 3));
    lineGeometry.setAttribute('color', new THREE.BufferAttribute(lineColors, 3));
    const lineMaterial = new THREE.LineBasicMaterial({vertexColors: true});
    const lines = new THREE.LineSegments(lineGeometry, lineMaterial);
    scene.add(lines);
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
