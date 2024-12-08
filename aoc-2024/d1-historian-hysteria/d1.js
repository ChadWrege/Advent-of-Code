const fs = require('fs');

// Specify the path of the text file
const filePath = '/home/chad/Repos/aoc-2024/d1-historian-hysteria/input.txt';

try {
  // Read the file synchronously
  const data = fs.readFileSync(filePath, 'utf8');

  // split data line by line
  const lines = data.split('\n');

  const left = [];
  const right = [];

  lines.forEach(line => {
    const collumn = line.split("   ");

    if (collumn.length===2) {
      left.push(Number(collumn[0]));
      right.push(Number(collumn[1]));
    }
  });

  // left.sort((a,b) => a - b);
  // right.sort((a,b) => a - b);

  // console.log("Left Side:", left);
  // console.log("Right Side", right);
  
  // const result = left.map((value, index) => {
  //   return Math.abs(value - right[index]);
  // });
  
  let result = 0;

  left.forEach(value => {
    const count = right.filter((item) => {
     return item === value  
    }).length;

    result = result + value * count;
  });

  //const sum = result.reduce((acc, value) => {
  //     return acc + value;
  //}, 0);

  console.log(result);

  // Print the contents of the file
  // console.log(data);
} catch (err) {
  console.error('Error reading file:', err);
}

