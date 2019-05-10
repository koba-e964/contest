#include <algorithm>
#include <cstdio>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;

int main(void) {
  int n,m;
  scanf("%d%d",&n,&m);
  printf("%d\n",min(max(1,m),n-m));
}
