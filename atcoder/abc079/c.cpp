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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string v;
  cin >> v;
  REP(a, 0, 2) {
    REP(b, 0, 2) {
      REP(c, 0, 2) {
	string ret = v.substr(0, 1) + (a == 0 ? "+" : "-")
	  + v.substr(1, 1) + (b == 0 ? "+" : "-")
	  + v.substr(2, 1) + (c == 0 ? "+" : "-")
	  + v.substr(3, 1) + "=7";
	int k = v[0] - '0';
	k += (a == 0 ? 1 : -1) * (v[1] - '0');
	k += (b == 0 ? 1 : -1) * (v[2] - '0');
	k += (c == 0 ? 1 : -1) * (v[3] - '0');
	if (k == 7) {
	  cout << ret << endl;
	  return 0;
	}
      }
    }
  }
}
