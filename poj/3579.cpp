#include <algorithm>
#include <cstdio>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

int main(void){
  int n;
  while(scanf("%d", &n) >= 1) {
    vector<int> a(n);
    REP(i, 0, n)
      scanf("%d", &a[i]);
    sort(a.begin(), a.end());
    int lo = 0, hi = a[n - 1] - a[0];
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      ll cnt = 0;
      REP(i, 0, n)
	cnt += a.end() - lower_bound(a.begin(), a.end(), a[i] + mid);
      ll comb = (ll) n * (n - 1) / 2;
      if (cnt <= comb - (comb + 1) / 2)
	hi = mid;
      else
	lo = mid;
    }
    printf("%d\n", lo);
  }
}
