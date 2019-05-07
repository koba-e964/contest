#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s,t;
  while(cin>>s>>t&&t!="X") {
    vector<string> words;
    int last = 0;
    REP(i, 0, s.size()) {
      if (s[i] == '_') {
        words.push_back(s.substr(last, i - last));
        last = i + 1;
      }
      if (s[i] <= 'Z' && i > 0) {
        words.push_back(s.substr(last, i - last));
        last = i;
      }
    }
    words.push_back(s.substr(last));
    if (t == "U" || t == "L") {
      REP(i, 0, words.size()) {
        if (i > 0 || t == "U") words[i][0] = toupper(words[i][0]);
        else words[i][0] = tolower(words[i][0]);
        cout << words[i];
      }
      cout << endl;
    } else {
      REP(i, 0, words.size()) {
        if (i > 0) cout << "_";
        transform(words[i].begin(), words[i].end(), words[i].begin(), ::tolower);
        cout << words[i];
      }
      cout << endl;
    }
  }
}
