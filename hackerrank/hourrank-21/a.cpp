#include <algorithm>
#include <iomanip>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, xx;
  cin >> n >> xx;
  VI x(n), v(n);
  REP(i, 0, n) cin >> x[i];
  REP(i, 0, n) cin >> v[i];
  vector<PI> t;
  REP(i, 0, n) {
    x[i] = abs((x[i] - xx) / v[i]);
    t.push_back(PI(x[i], i));
  }
  sort(t.begin(), t.end());
  if (n == 1 || t[0].first != t[1].first) {
    cout << t[0].second << endl;
  } else {
    cout << -1 << endl;
  }
}
