#include <iostream>
#include <cstdio>
#include <vector>
#include <string>
#include <algorithm>
using namespace std;

#define rep(i, n)       reps(i, 0, n)
#define reps(i, m, n)   for (int i = m; i < int(n); ++i)


const int N = 1<<20;
#define DEBUG 0

int n;
int g[N];
string cipher;

string modify(int bits) {
    string str = cipher;
    rep(j, n) {
        if(bits & (1<<(n-1-j))) {
            if(str[j] == 'z')
                return "";
            str[j]++;
        }
    }
    return str;
}
string invmod(int bits) {
    string str = cipher;
    rep(j, n) {
        if(bits & (1<<(n-1-j))) {
            if(str[j] == 'z')
                return "";
            str[j]++;
        }
    }
    return str;
}

bool check(string str) {
    reps(c, 'b', 'z'+1) {
        rep(i, str.size()) {
            if(str[i] == (char)c) {
                str[i] --;
                break;
            }
        }
    }
    return str == cipher;
}

void proc(void) {
    rep(i, 1<<n) {
        string str=invmod(i);
        if(str=="") continue;
        if(check(str))g[i]=1;
    }
}

int main()
{
    string str;
    while(1) {
        cin>>str;
        if(str=="#") break;
        cipher = str;
        rep(i,N)g[i]=0;
        n = cipher.size();
        proc();
        int count = 0;
        vector<int> ss;
        rep(i, 1<<n) {
            count += g[i];
            if(g[i]){
                ss.push_back(i);
            }
        }
        sort(ss.begin(),ss.end());
        if(DEBUG) {
            if(count <= 10)
            rep(k, count) {
                cout << k << "-" << ss[k] << ":" << modify(ss[k]) << endl;
            }
        }
        cout << count << endl;
        if( count <= 10) {
            rep(k, count) {
                cout << modify(ss[k]) << endl;
            }
        }
        else {
            rep(k,5) {
                cout << modify(ss[k]) << endl;
            }
            rep(k,5) {
                cout << modify(ss[ss.size()-5+k]) << endl;
            }
        }
    }
}

