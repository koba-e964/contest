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
const int N = 100100;

int x[N];
char d[N];

ll solve(VI ri, VI li) {
  ll s = 0;
  if (0) {
    cout << "ri: ";
    REP(i, 0, ri.size()) {
      cout << ri[i] << " ";
    }
    cout << endl;
    cout << "li: ";
    REP(i, 0, li.size()) {
      cout << li[i] << " ";
    }
    cout << endl;
  }
  if (ri.size() == 0 && li.size() == 0) {
    return 0;
  }
  ll piv;
  if (ri.size() > li.size()) {
    piv = li[0];
  } else {
    piv = ri[ri.size() - 1] + 1;
  }
  REP(i, 0, ri.size()) {
    s += piv - i - ri[i] - 1;
  }
  REP(i, 0, li.size()) {
    s += li[i] - (piv + i);
  }
  return s;
}

int main(void){
  int n,l;
  cin >> n >> l;
  REP(i, 0, n) {
    cin >> x[i] >> d[i];
    x[i]--;
  }
  ll sum = 0;
  int cnt = 0;
  int st = 0;
  int last = -1;
  ll acc = 0;
  vector<int> li, ri;
  REP(i, 0, n) {
    switch(st) {
    case 0:
      if (d[i] == 'L') {
	ri.push_back(-1);
	li.push_back(x[i]);
	st = 2;
      } else {
	st = 1;
	ri.push_back(x[i]);
	acc = x[i];
      }
      break;
    case 1:
      if (d[i] == 'L') {
	li.push_back(x[i]);
	st = 2;
      } else {
	ri.push_back(x[i]);
	st = 1;
      }
      break;
    case 2:
      if (d[i] == 'L') {
	li.push_back(x[i]);
	st = 2;
	last++;
      } else {
	st = 1;
	sum += solve(ri, li);
	li.clear();
	ri.clear();
	ri.push_back(x[i]);
      }
    }
  }
  if (ri.size() > 0 && li.size() == 0) {
    li.push_back(l);
  }
  sum += solve(ri, li);
  cout << sum << endl;
}
