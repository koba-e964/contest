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
  int n, l;
  cin >> n >> l;
  string s;
  cin >> s;
  int cur = 1;
  int crash = 0;
  REP(i, 0, n) {
    if (s[i] == '+') {
      cur++;
    } else {
      cur--;
    }
    if (cur > l) {
      crash++;
      cur = 1;
    }
  }
  cout << crash << endl;
}
