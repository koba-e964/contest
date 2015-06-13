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



int main(void){
  int n,r;
  string s;
  cin  >> n >> r >> s;
  int c = 0;
  int rm = n;
  int pos = 0;
  REP(i, 0, n) {
    if (s[i] == '.') {
      rm = i;
    }
  }
  REP(i, 0, n) {
    bool ok = true;
    REP(j,0,min(r, n - i)) {
      ok &= s[i+j] == 'o';
    }
    if (ok){
      continue;
    }
    if (i + r - 1 >= rm && rm >= i) {
      // i .. i + r - 1
      REP(j, 0, r) {
	s[i + j] = 'o';
      }
      c++;
      c += i - pos;      
      break;
    }
    if (s[i] == 'o') {
      continue;
    }
    // i .. i + r - 1
    REP(j, 0, r) {
      s[i + j] = 'o';
    }
    c++;
    c += i - pos;
    pos = i;
  }
  cout << c << endl;
}
