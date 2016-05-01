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

const int N = 101;

int t, n, m;
int a[N], b[N];

bool calc() {
  int cur = 0;
  REP(i, 0, N) {
    int rem = b[i];
    cur = max(cur, i - t);
    while (cur <= i && rem > 0) {
      int diff = min(a[cur], rem);
      a[cur] -= diff;
      rem -= diff;
      if (a[cur] == 0) {
	cur++;
      }
    }
    if (rem > 0) {
      return false;
    }
  }
  return true;
}

int main(void){
  cin >> t >> n;
  REP(i, 0, n) {
    int u;
    cin >> u;
    a[u]++;
  }
  cin >> m;
  REP(i, 0, m) {
    int u;
    cin >> u;
    b[u]++;
  }
  cout << (calc() ? "yes" : "no") << endl;
}
