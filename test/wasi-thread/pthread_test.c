#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <pthread.h>

void *thread_function(void *arg)
{
	int cnt = 10;
	while (cnt--) {
		printf("Hello from the new thread (%d)!\n", cnt);
		sleep(1);
	}
	return NULL;
}

int main()
{
	pthread_t my_thread;
	if (pthread_create(&my_thread, NULL, thread_function, NULL)) {
		printf("Error creating thread.\n");
		abort();
	}
	printf("Hello from the main thread!\n");
	if (pthread_join(my_thread, NULL)) {
		printf("Error joining thread.\n");
		abort();
	}
	return 0;
}
