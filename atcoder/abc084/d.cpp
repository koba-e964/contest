#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 100100;
int prime[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  REP(i, 2, N) {
    prime[i] = 1;
  }
  REP(i, 2, N) {
    if (prime[i] == 0) { continue; }
    for(int j=2;j<=(N-1)/i;++j) {
      prime[i * j] = 0;
    }
  }
  VI cond(N, 0);
  REP(i, 1, N) {
    if (i % 2 == 1 && prime[i] && prime[(i + 1) / 2]) {
      cond[i] = cond[i - 1] + 1;
    } else {
      cond[i] = cond[i - 1];
    }
  }
  int q;
  cin >> q;
  REP(_, 0, q) {
    int l, r;
    cin >> l >> r;
    cout << cond[r] - cond[l - 1] << "\n";
  }
}
