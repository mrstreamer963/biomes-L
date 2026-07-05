import { Container } from "pixi.js";

const MIN_ZOOM = 0.25;
const MAX_ZOOM = 3;

export class MapCamera {
  private container: Container;
  private canvas: HTMLCanvasElement;

  private isPanning = false;
  private panStart = { x: 0, y: 0 };
  private containerStart = { x: 0, y: 0 };

  constructor(container: Container, canvas: HTMLCanvasElement) {
    this.container = container;
    this.canvas = canvas;
    this.bindEvents();
  }

  private bindEvents() {
    this.canvas.addEventListener("pointerdown", this.onPointerDown);
    this.canvas.addEventListener("pointermove", this.onPointerMove);
    this.canvas.addEventListener("pointerup", this.onPointerUp);
    this.canvas.addEventListener("pointerleave", this.onPointerUp);
    this.canvas.addEventListener("wheel", this.onWheel, { passive: false });
  }

  destroy() {
    this.canvas.removeEventListener("pointerdown", this.onPointerDown);
    this.canvas.removeEventListener("pointermove", this.onPointerMove);
    this.canvas.removeEventListener("pointerup", this.onPointerUp);
    this.canvas.removeEventListener("pointerleave", this.onPointerUp);
    this.canvas.removeEventListener("wheel", this.onWheel);
  }

  private onPointerDown = (e: PointerEvent) => {
    this.isPanning = true;
    this.panStart = { x: e.clientX, y: e.clientY };
    this.containerStart = {
      x: this.container.position.x,
      y: this.container.position.y,
    };
    this.canvas.style.cursor = "grabbing";
  };

  private onPointerMove = (e: PointerEvent) => {
    if (!this.isPanning) return;
    const dx = e.clientX - this.panStart.x;
    const dy = e.clientY - this.panStart.y;
    this.container.position.set(
      this.containerStart.x + dx,
      this.containerStart.y + dy
    );
  };

  private onPointerUp = () => {
    this.isPanning = false;
    this.canvas.style.cursor = "grab";
  };

  private onWheel = (e: WheelEvent) => {
    e.preventDefault();

    const factor = e.deltaY > 0 ? 0.9 : 1.1;
    const newScale = Math.min(
      MAX_ZOOM,
      Math.max(MIN_ZOOM, this.container.scale.x * factor)
    );
    const actualFactor = newScale / this.container.scale.x;

    const rect = this.canvas.getBoundingClientRect();
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;

    this.container.position.set(
      mouseX - (mouseX - this.container.position.x) * actualFactor,
      mouseY - (mouseY - this.container.position.y) * actualFactor
    );
    this.container.scale.set(newScale);
  };
}
