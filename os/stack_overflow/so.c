#include <stdio.h>

void f(int depth, long int bottom) {
  printf("Depth = %d frame = %p\n", depth, &depth);
  printf("Bits allocated: %ld \n\n", bottom - (long)&depth);

  f(depth + 1, bottom);
}

int main(void) {
  int x = 0;

  f(x, (long)&x);
}
