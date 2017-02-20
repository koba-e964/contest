#include <cstdio>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;



int main(void){
  int n;
  while (scanf("%d", &n) == 1 && n) {
    VL v(n);
    ll grundy = 0;
    REP(i, 0, n) {
      scanf("%lld", &v[i]);
      grundy ^= v[i];
    }
    int cnt = 0;
    if (grundy != 0) {
      REP(i, 0, n) {
	ll next = v[i] ^ grundy;
	if (next < v[i]) {
	  cnt += 1;
	}
      }
    }
    printf("%d\n", cnt);
  }
}
