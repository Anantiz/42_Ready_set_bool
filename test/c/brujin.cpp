#include <stdio.h>
#include <math.h>
int main()
{
	u_int64_t i = 0;
	while (i < 8){
		u_int64_t j = 0;
		u_int64_t val = pow(4, i);
		printf("%lu  ", val);
		while (j < i){
			val += pow(4, j);
			printf("%lu  ", val);
			++j;
		}
	++i;}
	printf("\n");
}