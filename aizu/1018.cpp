#include <algorithm>
#include<stdio.h>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int main(void) {
  int n;
  for(;scanf("%d",&n)>0;){
    VI a(n);
    REP(i,0,n)scanf("%d",&a[i]);
    sort(a.begin(),a.end());
    int b=0;
    REP(i,0,n)b+=(n-i)*a[i];
    printf("%d\n",b);
  }
}
