#include <algorithm>
#include <iostream>
#include <queue>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 150000;


int main(void){
  int n, m;
  cin >> n >> m;
  vector<PI> men(n), women(m);
  REP(i, 0, n) {
    int a, b;
    cin >> a >> b;
    men[i] = PI(a, b);
  }
  REP(i, 0, m) {
    int c, d;
    cin >> c >> d;
    women[i] = PI(d, c);
  }
  sort(men.begin(), men.end());
  sort(women.begin(), women.end());
  int pos = 0;
  multiset<int> ms;
  int cnt = 0;
  REP (i, 0, n) {
    for (; pos < m && women[pos].first <= men[i].first; ++pos) {
      ms.insert(women[pos].second);
    }
    multiset<int>::iterator it = ms.lower_bound(men[i].second);
    if (it != ms.end()) {
      cnt++;
      ms.erase(it);
    }
  }
  cout << cnt << endl;
}
