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
  string s;
  cin >> s;
  ll wc = 0;
  ll sum = 0;
  REP(i, 0, s.length()) {
    switch(s[s.length() - i - 1]) {
    case 'w':
      wc++;
      break;
    case 'c':
      sum += wc * (wc - 1) / 2;
      break;
    default:
      ;
    }
  }
  cout << sum << endl;
}
