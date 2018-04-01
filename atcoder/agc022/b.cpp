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

int gcd(int x, int y) {
  while (y != 0) {
    int r = x % y;
    x = y;
    y = r;
  }
  return x;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI ans;
  if (n == 3) {
    ans = VI(3);
    ans[0] = 2;
    ans[1] = 5;
    ans[2] = 63;
  } else {
    if (n % 2 == 1) {
      ans.push_back(30000);
    }
    REP(i, 0, 500) {
      if ((int)ans.size() >= n) break;
      REP(j, 2, 30) {
	if (gcd(j, 30) != 1) {
	  ans.push_back(60 * i + j);
	  ans.push_back(60 * i + 60 - j);
	}
      }
      ans.push_back(60 * i + 30);
      ans.push_back(60 * i + 60);
    }
  }
  REP(i, 0, n) {
    cout << ans[i] << (i == n - 1 ? "\n" : " ");
  }
}
