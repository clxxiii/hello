#include <stdio.h>
#include <stdlib.h>

int *getcells(int n) {
  int *cells = malloc(sizeof(int) * n);

  for (int i = 0; i < n; i++) {
    fscanf(stdin, "%d", (cells + i));
  }
  return cells;
}

void cycle(int hops) { printf("cycle\n%d\n", hops); }
void magic(int hops) { printf("magic\n%d\n", hops); }
void left(int hops) { printf("left\n%d\n", hops); }
void right(int hops) { printf("right\n%d\n", hops); }

int main() {
  int n;
  int s;
  int m;
  if (fscanf(stdin, "%d %d %d", &n, &s, &m) != 3) {
    printf("Error when reading input\n");
    return 0;
  }

  int *cells = getcells(n);

  int frogpos = s;
  int hops = 0;
  while (1) {
    // printf("Frog on %d\n", frogpos);
    int *cell = (cells + (frogpos - 1));
    // printf("Cell number: %d\n", *cell);
    if (*cell == m) {
      break;
    }
    if (*cell == 0) {
      cycle(hops);
      return 0;
    }
    if (frogpos < 1) {
      left(hops);
      return 0;
    }
    if (frogpos > n) {
      right(hops);
      return 0;
    }
    frogpos = frogpos + *cell;
    *cell = 0;
    hops++;
  }
  magic(hops);
  free(cells);
}
