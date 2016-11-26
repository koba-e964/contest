#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 100100;
int x[N];

int solve(const VI& a, const VI& b) {
  if (a.size() < b.size()) {
    return solve(b, a);
  }
  // |a| >= |b|
  int s = b.size();
  int rem = a.size() - s;
  int idpair = 0;
  int cur = -1;
  REP(i, 0, a.size()) {
    if (a[i] == cur) {
      idpair++;
      cur = -1;
    } else {
      cur = a[i];
    }
  }
  return rem <= 2 * idpair ? s + rem / 2 : s + idpair;
}

int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, n) {
    cin >> x[i];
  }
  sort(x, x + n);
  vector<VI> res(m);
  REP(i, 0, n) {
    res[x[i] % m].push_back(x[i]);
  }
  int cnt = res[0].size() / 2;
  if (m % 2 == 0) {
    cnt += res[m / 2].size() / 2;
  }
  REP(i, 1, (m + 1) / 2) {
    cnt += solve(res[i], res[m - i]);
  }
  cout << cnt << endl;
}
