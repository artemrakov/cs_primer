#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
  printf("hello world (pid:%d)\n", (int)getpid());
  int rc = fork();
  int test_value = 100;
  if (rc < 0) {
    // fork failed
    fprintf(stderr, "fork failed\n");
    exit(1);
  } else if (rc == 0) {
    int test_value = 200;

    printf("hello, I am child (pid:%d) (value:%d)\n", (int)getpid(),
           test_value);
  } else {
    // int rc_wait = wait(NULL);
    int test_value = 300;
    // parent goes down this path (main)
    printf("hello, I am parent of %d (value:%d) (pid:%d)\n", rc, test_value,
           (int)getpid());
  }

  return 3;
}
