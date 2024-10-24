#include <stdio.h>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

int main(void){
    int n;
    scanf("%d", &n);
    REP(i, 0, 2 * n + 1) {
        REP(j, 0, 2 * n + 1) {
            printf("%c", i % 2 == 0 && j % 2 == 0 ? '.' : '#');
        }
        puts("");
    }
}
