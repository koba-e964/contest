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



int main(void){
  vector<string> st;
  int cnt = 0;
  string s;
  while (cin >> s) {
    if (s == "not") {
      cnt += 1;
    } else {
      REP(i, 0, cnt % 2) {
	st.push_back("not");
      }
      cnt = 0;
      st.push_back(s);
    }
  }
  REP(i, 0, cnt) {
    st.push_back("not");
  }
  REP(i, 0, st.size()) {
    cout << st[i] << (i == st.size() - 1 ? "\n" : " ");
  }
}
