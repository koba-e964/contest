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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  while (cin >> n && n) {
    VI a(n);
    REP(i, 0, n) cin >> a[i];
    int st = -1;
    int len = 0;
    vector<PI> cont;
    REP(i, 0, n) {
      if (a[i] == st + len) {
        len += 1;
      } else {
        if (st >= 0)
          cont.push_back(PI(st, len));
        st = a[i];
        len = 1;
      }
    }
    cont.push_back(PI(st, len));
    REP(i, 0, cont.size()) {
      if (cont[i].second == 1) {
        cout << cont[i].first;
      } else {
        cout << cont[i].first << "-" << cont[i].first + cont[i].second - 1;
      }
      cout << (i + 1 == cont.size() ? "\n" : " ");
    }
  }
}
