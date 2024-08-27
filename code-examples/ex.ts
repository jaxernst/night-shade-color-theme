// Define a type for RGB color
type RGB = [number, number, number];

interface Shape {
  color: RGB;
  area(): number;
}

class Circle implements Shape {
  constructor(
    public color: RGB,
    private radius: number
  ) {}

  area(): number {
    return Math.PI * this.radius ** 2;
  }
}

// Create a color palette from a base color
function createPalette(baseColor: RGB, variations: number): RGB[] {
  return Array.from({ length: variations }, (_, i) => {
    const factor = 1 - i / variations;
    return baseColor.map((c) => Math.round(c * factor)) as RGB;
  });
}

// Example usage
const baseColor: RGB = [255, 100, 50];
const palette = createPalette(baseColor, 5);

const circle = new Circle(palette[2], 10);
console.log(`Circle area: ${circle.area().toFixed(2)}`);
console.log(`Circle color: rgb(${circle.color.join(",")})`);
