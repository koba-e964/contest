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

int n;
int a[6] = {};

void dfs(int v) {
  if (v == n) {
    string t(n, '*');
    REP(i, 0, n) {
      t[i] = '0' + a[i];
    }
    cout << t << endl;
    return;
  }
  dfs(v + 1);
  assert (a[v] == 0 || a[v] == 9);
  bool inc = a[v] == 0;
  REP(i, 0, 9) {
    if (inc) {
      a[v] = (a[v] + 1) % 10;
    } else {
      a[v] = (a[v] + 9) % 10;
    }
    dfs(v + 1);
  }
}

int main(void){
  cin >> n;
  int m = 1;
  REP(i, 0, n) { m *= 10; }
  cout << m - 1 << endl;
  dfs(0);
}
