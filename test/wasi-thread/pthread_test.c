#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <pthread.h>

/*
 * Max thread number is defined as CLUSTER_MAX_THREAD_NUM (default: 4) in WAMR,
 */
//#define THREAD_NUM 4
#define THREAD_NUM 10

void *thread_function(void *arg)
{
	int index = *(int *)arg;
	int cnt = 10;
	while (cnt--) {
		printf("Hello from the new thread (%d)!\n", index);
		sleep(1);
	}
	return NULL;
}

int main()
{
	pthread_t threads[THREAD_NUM];
	int i;

	printf("Hello from the main thread!\n");

	for (i = 0; i < THREAD_NUM; i++) {
		if (pthread_create(&threads[i], NULL, thread_function, &i)) {
			printf("Error creating thread.\n");
			abort();
		}
	}

	for (i = 0; i < THREAD_NUM; i++) {
		if (pthread_join(threads[i], NULL)) {
			printf("Error joining thread.\n");
			abort();
		}
	}
	return 0;
}
