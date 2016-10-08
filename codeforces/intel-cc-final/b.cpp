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

const int N = 20;
int a[N][N];
int n, m;

bool check() {
  REP(i, 0, n) {
    int cnt = 0;
    REP(j, 0, m) {
      cnt += j + 1 == a[i][j] ? 0 : 1;
    }
    if (cnt > 2) {
      return 0;
    }
  }
  return 1;
}

int main(void){
  cin >> n >> m;
  REP(i, 0, n) {
    REP(j, 0, m) {
      cin >> a[i][j];
    }
  }
  REP(i, 0, m) {
    REP(j, 0, i) {
      // swap i, j
      REP(k, 0, n) {
	swap(a[k][i], a[k][j]);
      }
      bool ok = check();
      if (ok) {
	cout << "YES" << endl;
	return 0;
      }
      // swap i, j again
      REP(k, 0, n) {
	swap(a[k][i], a[k][j]);
      }
    }
  }
  cout << (check() ? "YES" : "NO") << endl;
}
