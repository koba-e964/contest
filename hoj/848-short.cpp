#include <algorithm>
#include <iostream>
#include <vector>
#define R for(int i=1;i<=n;++i)
using namespace std;
typedef long long ll;
typedef pair<int, int> PI;
typedef pair<ll, int> PLI;
ll a;
int main(void) {
  int n;
  ll c;
  cin >> n >> c;
  vector<PLI> pool;
  R cin >> a,pool.push_back(PLI(a,i));
  sort(pool.begin(), pool.end());
  pair<ll, PI> mi(5e15,PI(1e8,1e8));
  R {
    cin >> a;
    if (!a)
      mi = min(mi, make_pair(c, PI(i, 1)));
    else {
      vector<PLI>::iterator it =
	lower_bound(pool.begin(), pool.end(), PLI((c+a-1)/a, -1));
#define UPD mi = min(mi, make_pair(abs(c - a * v), PI(idx, i)));
      if (it != pool.begin()) {
	ll v = (it - 1)->first;
	int idx = (it - 1)->second;
	UPD
      }
      if (it != pool.end()) {
	ll v = it->first;
	int idx = it->second;
	UPD
      }
    }
  }
  cout << mi.second.first << " " << mi.second.second << endl;
}
