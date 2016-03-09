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

const int inf = 10000;
int min_cww(string s) {
  int cc = 0, wc = 0;
  REP(i, 0, s.length()) {
    switch (s[i]) {
    case 'c':
      cc++;
      break;
    case 'w':
      if(cc >= 1) wc++;
      break;
    default:
      ;
    }
    if (cc >= 1 && wc >= 2) {
      return i + 1;
    }
  }
  return inf;
}

int main(void){
  string s;
  cin >> s;
  int mi = inf;
  REP(i, 0, s.length()) {
    mi = min(mi, min_cww(s.substr(i)));
  }
  cout << (mi == inf ? -1 : mi) << endl;
}
