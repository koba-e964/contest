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
int numsub[N], bad[N];
int a[N];
int inc[N], de[N];


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n) cin >> a[i];
  inc[n - 1] = 1;
  de[n - 1] = 1;
  for (int i = n - 2; i >= 0; --i) {
    if (a[i] <= a[i + 1]) {
      inc[i] = inc[i + 1] + 1;
      de[i] = 1;
    } else {
      inc[i] = 1;
      de[i] = de[i + 1] + 1;
    }
  }
  REP(minrun, 2, n + 1) {
    int s = 0, b = 0;
    int pos = 0;
    while (pos < n) {
      int u = max(inc[pos], de[pos]);
      if (u < minrun) {
        int oldu = u;
        u = min(n - pos, minrun);
        b += u - oldu;
      }
      s += 1;
      pos += u;
    }
    assert (pos <= n);
    numsub[minrun] = s;
    bad[minrun] = b;
  }
  int q;
  cin >> q;
  REP(_, 0, q) {
    int minrun;
    cin >> minrun;
    cout << numsub[minrun] << " " << bad[minrun] << endl;
  }
}
