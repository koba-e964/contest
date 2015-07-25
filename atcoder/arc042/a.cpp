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

const int M = 200000;
int a[M];

int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    cin >> a[i];
  }
  set<int> p;
  vector<int> q;
  for (int i = m - 1; i >= 0; --i) {
    if (p.count(a[i])) {
      continue;
    }
    q.push_back(a[i]);
    p.insert(a[i]);
  }
  for (int i = 0; i < n; ++i) {
    if (p.count(i + 1) == 0) {
      q.push_back(i + 1);
    }
  }
  REP(i, 0, n) {
    cout << q[i] << endl;
  }
}
