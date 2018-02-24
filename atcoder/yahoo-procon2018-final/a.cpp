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


int gcd(int x,int y) {
  while (y != 0) {
    int r = x % y;
    x = y; y = r;
  }
  return x;
}

const int N = 100100;
const int W = 330;
int f[N];
int conv[N];
int pr[N];
int fac[N];
int ans[N];
int vis[N];
int prec[1000];


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
    //a[i] = i + 1;
    f[a[i]] += 1;
  }
  REP(i, 1, N) {
    for (int j = 1; j * i < N; ++j) {
      conv[i] += f[i * j];
    }
  }
  REP(i, 2, N) {
    pr[i] = 1;
  }
  REP(i, 2, N) {
    if (not pr[i]) continue;
    fac[i] = i;
    for (int j = 2; j * i < N; ++j) {
      pr[i * j] = 0;
      fac[i * j] = i;
    }
  }
  REP(i, 2, N) {
    if (i % 4 == 0) {
      ans[i] = ans[i / 2];
      continue;
    }
    if ((i / fac[i]) % fac[i] != 0) {
      int tmp = i == fac[i] ? 0 : ans[i / fac[i]];
      tmp += conv[fac[i]];
      if (i != fac[i]) {
	int v = i / fac[i];
	for (int j = 2; j * fac[i] < N; ++j) {
	  if (gcd(j, v) != 1) {
	    tmp-=f[fac[i]*j];
	  }
	}
      }
      ans[i] = tmp;
      continue;
    }
    ans[i] = ans[i / fac[i]];
  }
  if (0) {
    REP(i, 1, 10) {
      cerr<<" "<<ans[i];
    }
    cerr<<endl;
  }
  REP(i, 1, m + 1) {
    cout << n - ans[i] << "\n";
  }
}
