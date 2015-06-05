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
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 100010;
int a[N];
int n;

/* is 0 constructible in [0, p)? */
int rec(int p, stringstream &ss) {
  if (p == 0) {
    return 0;
  }
  if (a[p-1]) {
    return 0;
  }
  int k = p - 1;
  while (k >= 0 && a[k] == 0) {
    k--;
  }
  k++;
  if (p - k >= 3) {
    REP (i, 0, p - 3) {
      ss << a[i];
      ss << "->";
    }
    ss << "(0->0)->0";
    return 1;
  }
  if (p - k == 1) {
    REP (i, 0, p) {
      ss << a[i];
      if (i != p - 1) {
	ss << "->";
      }
    }
    return 1;
  }
  int u = k - 1;
  while (u >= 0 && a[u] == 1) {
    u--;
  }
  u ++;
  int sub = rec(u, ss);
  if (sub) {
    ss << "->(";
    REP (ttt, u, k) {
      ss << "1->";
    }
    ss << "0)->0";
    return 1;
  }
  return 0;
}

int main(void){
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  stringstream ss;
  if ( rec(n, ss)) {
    cout << "YES" << endl;
    cout << ss.str() << endl;
  } else {
    cout << "NO" << endl;
  }
}
