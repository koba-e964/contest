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
int n;
int p[N];
int nc[N] = {};
VI child[N];

int main(void){
  cin >> n;
  REP(i, 1, n) {
    cin >> p[i];
    child[p[i]].push_back(i);
  }
  for (int i = n - 1; i >= 0; --i) {
    nc[i]++;
    nc[p[i]] += nc[i];
  }
  REP(i, 0, n) {
    int ma = 0;
    if (i != 0) {
      ma = n - nc[i];
    }
    REP(j, 0, child[i].size()) {
      int v = child[i][j];
      ma = max(ma, nc[v]);
    }
    cout << ma << endl;
  }
}
