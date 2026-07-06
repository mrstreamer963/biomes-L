import { Container } from "pixi.js";

const MIN_ZOOM = 0.25;
const MAX_ZOOM = 3;
const PAN_THRESHOLD = 5;

export type ClickCallback = (screenX: number, screenY: number) => void;

export class MapCamera {
  private container: Container;
  private canvas: HTMLCanvasElement;

  private isPanning = false;
  private wasDrag = false;
  private pointerDown: { x: number; y: number } | null = null;
  private panStart = { x: 0, y: 0 };
  private containerStart = { x: 0, y: 0 };
  private onClickCb: ClickCallback | null = null;

  onPointerUp: ((e: PointerEvent) => void) | null = null;

  constructor(container: Container, canvas: HTMLCanvasElement) {
    this.container = container;
    this.canvas = canvas;
    this.bindEvents();
  }

  onClick(cb: ClickCallback) {
    this.onClickCb = cb;
  }

  private bindEvents() {
    this.canvas.addEventListener("pointerdown", this.handlePointerDown);
    this.canvas.addEventListener("pointermove", this.handlePointerMove);
    this.canvas.addEventListener("pointerup", this.handlePointerUp);
    this.canvas.addEventListener("pointerleave", this.handlePointerUp);
    this.canvas.addEventListener("wheel", this.onWheel, { passive: false });
  }

  destroy() {
    this.canvas.removeEventListener("pointerdown", this.handlePointerDown);
    this.canvas.removeEventListener("pointermove", this.handlePointerMove);
    this.canvas.removeEventListener("pointerup", this.handlePointerUp);
    this.canvas.removeEventListener("pointerleave", this.handlePointerUp);
    this.canvas.removeEventListener("wheel", this.onWheel);
    this.onClickCb = null;
  }

  private handlePointerDown = (e: PointerEvent) => {
    this.wasDrag = false;
    this.pointerDown = { x: e.clientX, y: e.clientY };
    this.panStart = { x: e.clientX, y: e.clientY };
    this.containerStart = {
      x: this.container.position.x,
      y: this.container.position.y,
    };
    this.isPanning = false;
  };

  private handlePointerMove = (e: PointerEvent) => {
    if (this.isPanning) {
      const dx = e.clientX - this.panStart.x;
      const dy = e.clientY - this.panStart.y;
      this.container.position.set(
        this.containerStart.x + dx,
        this.containerStart.y + dy
      );
    } else if (this.pointerDown) {
      const dx = e.clientX - this.pointerDown.x;
      const dy = e.clientY - this.pointerDown.y;
      if (Math.sqrt(dx * dx + dy * dy) > PAN_THRESHOLD) {
        this.isPanning = true;
        this.wasDrag = true;
        this.panStart = { x: e.clientX, y: e.clientY };
        this.containerStart = {
          x: this.container.position.x,
          y: this.container.position.y,
        };
        this.canvas.style.cursor = "grabbing";
      }
    }
  };

  private handlePointerUp = (e: PointerEvent) => {
    this.isPanning = false;
    this.pointerDown = null;
    if (!this.wasDrag) {
      const rect = this.canvas.getBoundingClientRect();
      const sx = e.clientX - rect.left;
      const sy = e.clientY - rect.top;
      this.onClickCb?.(sx, sy);
    }
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
