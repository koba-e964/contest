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
#include <stack>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int H = 2437;
string s[H];

struct Rectangle{ ll height;  int pos; };

// http://algorithms.blog55.fc2.com/blog-entry-132.html
int solve(const VI &histo) {
  int n = histo.size();
  stack<Rectangle> S;
  VL buffer(n + 1);
  ll maxv = 0;
  buffer[n] = 0;
  REP(i, 0, n) {
    buffer[i] = histo[i];
  }
     
     for ( int i = 0; i <= n; i++ ){
         Rectangle rect;
         rect.height = buffer[i];
         rect.pos = i;
         if ( S.empty() ){
             S.push( rect );
         } else {
             if ( S.top().height < rect.height ){
                 S.push( rect );
             } else if ( S.top().height > rect.height ) {
                 int target = i;
                 while ( !S.empty() && S.top().height >= rect.height){
                     Rectangle pre = S.top(); S.pop();
                     ll area = pre.height * (i - pre.pos + 1);
                     maxv = max( maxv, area);
                     target = pre.pos;
                 }
                 rect.pos = target;
                 S.push(rect);
             }
         }
     }
     
     return maxv;
}


int main(void) {
  int h, w;
  cin >> h >> w;
  REP(i, 0, h) {
    cin >> s[i];
  }
  vector<VI> diff(h - 1, VI(w - 1, 0));
  REP(i, 1, h) {
    REP(j, 1, w) {
      diff[i - 1][j - 1] =
	s[i][j] ^ s[i - 1][j] ^ s[i][j - 1] ^ s[i - 1][j - 1] ? 1 : 0;
    }
  }
  int ma = max(h, w);
  vector<VI> histo(h - 1, VI(w - 1, 0));
  REP(j, 0, w - 1) {
    int cur = 1;
    REP(i, 0, h - 1) {
      if (diff[i][j] == 1) {
	cur = 1;
      } else {
	cur += 1;
      }
      histo[i][j] = cur;
    }
  }
  REP(i, 0, h - 1) {
    if (0) {
      cerr << "histo[" << i <<"]:";
      REP(j, 0, w - 1) {
	cerr << " " << histo[i][j];
      }
      cerr << endl;
    }
    ma = max(ma, solve(histo[i]));
  }
  if (0) {
    REP(i, 0, h - 1) {
      REP(j, 0, w - 1) {
	cerr << diff[i][j] << " ";
      }
      cerr << endl;
    }
  }
  cout << ma << endl;
}
