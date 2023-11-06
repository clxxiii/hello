#include <stdio.h>

int main() {
  int ticks;
  fscanf(stdin, "%d", &ticks);
  float turns = ticks / 4.0f;
  printf("%f", turns);
  return 0;
}
