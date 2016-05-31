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
  int n;
  string s;
  cin >> n >> s;
  string cur = "b";
  REP(i, 0, 53) {
    if (cur == s) {
      cout << i << endl;
      return 0;
    }
    switch (i % 3) {
    case 0:
      cur = "a" + cur + "c";
      break;
    case 1:
      cur = "c" + cur + "a";
      break;
    case 2:
      cur = "b" + cur + "b";
      break;
    }
  }
  cout << -1 << endl;
  return 0;
}
