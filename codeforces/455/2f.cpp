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

void solve1(VI &a, int r) {
  if (r <= 0) { return; }
  int c = 1;
  while (r + 2 > c) { c *= 2; }
  c -= 2;
  REP(i, c - r, r) {
    a[i] = c - i;
  }
  solve1(a, c - r);
}

void solve2(VI &a, int r) {
  assert (r >= 6);
  if (r == 6) {
    int magic[6] = {5, 3, 1, 6, 4, 2};
    REP(i, 0, 7) {
      a[i] = magic[i];
    }
    return;
  }
  if (r == 7) {
    int magic[7] = {3, 6, 1, 5, 4, 7, 2};
    REP(i, 0, 7) {
      a[i] = magic[i];
    }
    return;
  }
  int c = 1;
  while (r > c) { c *= 2; }
  c /= 2;
  assert (r > c);
  // construct a derangement of [c, r]
  REP(i, c - 1, r) {
    a[i] = c + ((i - c + 1 + 1) % (r - c + 1));
  }
  solve2(a, c - 1);
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  // 1
  {
    if (n % 2 == 1) {
      cout << "NO\n";
    } else {
      VI ans1(n);
      solve1(ans1, n);
      // assertion
      REP(i, 0, n) {
        assert ((ans1[i] & (i + 1)) == 0);
      }
      cout << "YES\n";
      REP(i, 0, n) {
	cout << ans1[i] << (i == n - 1 ? "\n" : " ");
      }
    }
  }
  // 2
  {
    if ((1 <= n && n <= 5) || (n & -n) == n) {
      cout << "NO\n";
    } else {
      VI ans2(n);
      solve2(ans2, n);
      // assertion
      REP(i, 0, n) {
	assert ((ans2[i] & (i + 1)) != 0 && ans2[i] != i + 1);
      }
      cout << "YES\n";
      REP(i, 0, n) {
	cout << ans2[i] << (i == n - 1 ? "\n" : " ");
      }
    }
  }

#if 0
  // exp
  VI a(n);
  REP(i, 0, n) {
    a[i] = i + 1;
  }
  do {
    bool ok = true;
    REP(i, 0, n) {
      if ((a[i] & (i + 1)) == 0 || a[i] == i + 1) {
	ok = false;
	break;
      }
    }
    if (ok) {
      cerr << "answer of 1.\n";
      REP(i, 0, n) {
	cerr << " " << a[i];
      }
      cerr << "\n";
    }
  } while (next_permutation(a.begin(), a.end()));
#endif
}
