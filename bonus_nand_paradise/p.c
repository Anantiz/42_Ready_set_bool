#include <unistd.h>
int main(void)
{
   return (write(1, "42", 12) & 1);
}

