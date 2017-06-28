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



int main(void){
  cin.tie(0);
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  VI p(n);
  REP(i, 0, n) {
    cin >> p[i];
    p[i]--;
  }
  ll tot = 0;
  ll inc = 0;
  ll dec = 0;
  REP(i, 0, n) {
    tot += abs(p[i] - i);
    if (p[i] > i) {
      dec += 1;
    } else {
      inc += 1;
    }
  }
  vector<VI> quake(n, VI());
  REP(i, 0, n) {
    if (p[i] > i) {
      quake[p[i] - i - 1].push_back(i);
    } else if (p[i] < i) {
      quake[p[i] - i + n - 1].push_back(i);
    }
    quake[n - i - 1].push_back(i);
  }
  ll mi = tot;
  int mini = 0;
  REP(i, 1, n) {
    if (0) {
      cerr << "i = " << i << ", "
	   << mi << " " << mini << "\n";
      cerr << "(inc, dec) = (" << inc << ", " << dec << ")\n";
      cerr << "tot = " << tot << "\n";
    }
    tot += inc - dec;
    // increment
    bool judex = false; 
    REP(j, 0, quake[i - 1].size()) {
      int w = quake[i - 1][j];
      if (w + i == n && not judex) {
	inc -= 1;
	dec += 1;
	tot += -(n - p[w]) + (p[w] - 0);
	judex = true;
      } else {
	inc += 1;
	dec -= 1;
      }
    }
    if (mi > tot) {
      mi = tot;
      mini = i;
    }
  }
  cout << mi << " " << mini << "\n";
}
