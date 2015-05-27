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

int enc(string s) {
  return (s[0] == 'o' ? 4 : 0)
    + (s[1] == 'o' ? 2: 0)
    + (s[2] == 'o' ? 1: 0);
}

int main(void){
  string s, t;
  ll n;
  cin >> s >> n >> t;
  int p1 = count(s.begin(), s.end(), 'o');
  int p2 = count(t.begin(), t.end(), 'o');
  if (p1 != p2) {
    cout << "SUCCESS" << endl;
    return 0;
  }
  if (n >= 2) {
    cout << "FAILURE" << endl;
    return 0;
  }
  if (n == 0) {
    cout << (s == t ? "FAILURE" : "SUCCESS") << endl;
    return 0;
  }
  int se = enc(s), te = enc(t);
  int taboo[] = {18,12,33,45,30,51};
  if (count(taboo, taboo + 6, se * 8 + te)) {
    cout << "SUCCESS" << endl;
  } else {
    cout << "FAILURE" << endl;
  }
}
