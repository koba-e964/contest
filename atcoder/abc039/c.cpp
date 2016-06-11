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
  string board = "WBWBWWBWBWBWWBWBWWBW";
  int tt[7] = {0, 2, 4, 5, 7, 9, 11};
  string res[7] = {"Do", "Re", "Mi", "Fa", "So", "La", "Si"};
  REP(i, 0, 7) {
    string t = board.substr(tt[i], 12 - tt[i]);
    t += board.substr(0, tt[i]);
    if (s.substr(0, 12) == t) {
      cout << res[i] << endl;
      return 0;
    }
  }
}
