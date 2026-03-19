const fs = require('fs');
const { createCanvas } = require('canvas');

// Create a 1024x1024 icon
const size = 1024;
const canvas = createCanvas(size, size);
const ctx = canvas.getContext('2d');

// Background gradient
const gradient = ctx.createLinearGradient(0, 0, size, size);
gradient.addColorStop(0, '#4a90d9');
gradient.addColorStop(1, '#2a5090');
ctx.fillStyle = gradient;
ctx.fillRect(0, 0, size, size);

// Draw PDF document shape
ctx.fillStyle = '#ffffff';
ctx.beginPath();
ctx.moveTo(size * 0.2, size * 0.15);
ctx.lineTo(size * 0.6, size * 0.15);
ctx.lineTo(size * 0.8, size * 0.35);
ctx.lineTo(size * 0.8, size * 0.85);
ctx.lineTo(size * 0.2, size * 0.85);
ctx.closePath();
ctx.fill();

// Draw fold
ctx.fillStyle = '#e0e0e0';
ctx.beginPath();
ctx.moveTo(size * 0.6, size * 0.15);
ctx.lineTo(size * 0.6, size * 0.35);
ctx.lineTo(size * 0.8, size * 0.35);
ctx.closePath();
ctx.fill();

// Draw PDF text
ctx.fillStyle = '#4a90d9';
ctx.font = `bold ${size * 0.15}px Arial`;
ctx.textAlign = 'center';
ctx.textBaseline = 'middle';
ctx.fillText('PDF', size * 0.5, size * 0.55);

// Save the icon
const buffer = canvas.toBuffer('image/png');
fs.writeFileSync('app-icon.png', buffer);
console.log('Icon created successfully: app-icon.png');
