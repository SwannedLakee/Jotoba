<!DOCTYPE html>
<html>
   <head>
      <title>Jisho</title>
      <link rel="stylesheet" type="text/css" href="assets/css/lib/bootstrap.min.css">
      <link rel="stylesheet" type="text/css" href="assets/css/main.css">
      <link rel="stylesheet" type="text/css" href="assets/css/searchBar.css">
      <script src="assets/js/lib/jquery.min.js"></script>
      <script defer src="assets/js/lib/bootstrap.min.js"></script>
      <script defer src="assets/js/lib/choices.js"></script>
   </head>

   <body>

      <!-- Search Row -->
      <div id="search-row" class="wrap-row">
         <div class="d-flex center">

            <!-- Left Buttons -->
            <div class="btn-container">
               <svg height="30" width="30">
                  <path d="M22.996,25.811L4.12,25.816L4.124,6.942L17.34,6.937l2.07-2.195H2.997c-0.611,0-1.108,0.496-1.108,1.108v21.064   c0,0.613,0.497,1.109,1.108,1.109h21.063c0.61,0,1.105-0.496,1.105-1.109V11.329l-2.23,2.192L22.996,25.811z M26.798,3.291   C24.878,1.37,23.9,2.123,23.9,2.123L9.488,16.534L7.679,22.41l5.875-1.811L27.965,6.19C27.965,6.19,28.721,5.213,26.798,3.291z    M10.33,20.861l-0.918-1.286l0.745-2.312l2.392,2.854L10.33,20.861z"/>
               </svg>
               <h4>Zeichnen</h4>
            </div>
            <div class="btn-container">
               <svg height="30" width="30">
                  <path d="M11.986,13.132H2.599v1.653h14.053v-1.653H12.68c0.613-1.333,1.28-3.253,1.679-4.559c0.321-0.027,0.507-0.107,0.561-0.268   L12.866,7.72c-0.374,1.521-1.092,3.759-1.705,5.146L11.986,13.132L11.986,13.132z M5.026,8.306c0.561,1.387,1.041,3.228,1.172,4.4   l1.573-0.373c-0.159-1.174-0.666-2.987-1.28-4.347L5.026,8.306L5.026,8.306z M6.146,27.4V25.88h7.121v1.359h1.734v-9.974H4.44V27.4   H6.146L6.146,27.4z M13.266,24.279H6.145v-5.413h7.121V24.279z M10.573,5.719V3.105c0.347-0.052,0.507-0.186,0.533-0.373L8.838,2.6   v3.119H3.292v1.6H16.2v-1.6H10.573L10.573,5.719z M25.88,4.199h-8.187v23.174h1.729V5.908h5.311   c-0.907,2.159-2.053,4.667-3.36,7.306c2.909,2.667,4.214,4.534,4.214,6.267c0,2.453-1.279,2.533-3.066,2.533   c-0.506,0-1.063-0.022-1.647-0.08c0.346,0.508,0.584,1.281,0.61,1.762c0.4,0.027,0.826,0.027,1.229,0.027   c1.731,0,4.642,0,4.642-4.188c0-1.921-1.199-3.92-4.08-6.614c1.467-2.8,2.881-5.679,3.896-7.946   c0.106-0.053,0.214-0.133,0.239-0.267L26.23,4.12L25.885,4.2L25.88,4.199L25.88,4.199z"/>
               </svg>
               <h4>Radikale</h4>
            </div>
            <div class="btn-container">
               <svg height="30" width="30">
                  <path d="M15.93,24.714v3.091c3.09,0.168,5.483,1.19,5.483,2.146c0,0.023-12.679,0.023-12.679,0c0-0.919,2.213-1.91,5.139-2.133   v-3.115c-3.734-0.535-6.616-3.717-6.616-7.602v-1.541h1.027v1.541c0,3.688,2.992,6.682,6.681,6.683h0.005   c3.691-0.001,6.743-2.991,6.743-6.683v-1.541h1.029v1.541C22.743,21.031,19.729,24.236,15.93,24.714L15.93,24.714z M14.965,22.693   c-3.122,0-5.654-2.532-5.654-5.654v-1.477h11.373v1.477C20.684,20.161,18.087,22.693,14.965,22.693z M9.312,5.685   c0-3.121,2.531-5.653,5.654-5.653s5.718,2.532,5.718,5.653v8.849H9.312V5.685z"></path>
               </svg>
               <h4>Sprechen</h4>
            </div>

            <!-- Search Bar -->
            <div id="searchDiv">
               <div class="searchDivInner">
                  <form>
                     <div class="inner-form">
                        <div class="input-field first-wrap">
                           <div class="input-select">
                              <select data-trigger="" name="choices-single-default">
                                 <option>Wörter & Kanji</option>
                                 <option>Sätze</option>
                                 <option>Namen</option>
                              </select>
                           </div>
                        </div>
                        <div class="input-field second-wrap">
                           <input id="search" type="text" placeholder="Deutsch, Japanisch, Romaji, Wörter oder Text" tabindex="1" lang="ja" autocapitalize="off" autocomplete="off" data-autoload="false" data-effective-keyword=""/>
                        </div>
                        <div class="input-field third-wrap">
                           <button class="btn-search" type="button">
                              <svg class="svg-inline--fa fa-search fa-w-16" aria-hidden="true" data-prefix="fas" data-icon="search" role="img" viewBox="0 0 512 512">
                                 <path fill="white" d="M505 442.7L405.3 343c-4.5-4.5-10.6-7-17-7H372c27.6-35.3 44-79.7 44-128C416 93.1 322.9 0 208 0S0 93.1 0 208s93.1 208 208 208c48.3 0 92.7-16.4 128-44v16.3c0 6.4 2.5 12.5 7 17l99.7 99.7c9.4 9.4 24.6 9.4 33.9 0l28.3-28.3c9.4-9.4 9.4-24.6.1-34zM208 336c-70.7 0-128-57.2-128-128 0-70.7 57.2-128 128-128 70.7 0 128 57.2 128 128 0 70.7-57.2 128-128 128z"></path>
                              </svg>
                           </button>
                        </div>
                     </div>
                  </form>
                  <div>
                  </div>
               </div>
            </div>
         </div>
      </div>

      <!-- Main Body -->
      <div id="page-container"> 
        <div class="d-flex center">
          <div class="main-container">

            <!-- Inner Top Row -->
            <div class="d-flex center no-drag">
              <div class="main-tab-select l">
                <h2 id="previousTabBtn" class="tab-btn">&larr; Wörter</h2>
              </div>
              <div class="title-div">
                <h1>Namen</h1>
                <h4>1070 gefunden</h4>
              </div>
              <div class="main-tab-select r">
                <h2 id="nextTabBtn" class="tab-btn">Sätze &rarr;</h2>
              </div>
            </div>
            <br>
            <!-- Inner Main Body -->
            <div class="d-flex center">
              <div class="main-info d-flex flex-column">

                   <!-- Entry -->

                  <!-- !!! Start of Name Template  !!! -->
                  <div class="list-entry">
                     <div class="d-flex flex-row">
                        <div class="kanji-preview small">
                           たなかいっこう
                        </div>
                        <div class="kanji-preview small">
                           【田中一光】
                        </div>
                     </div>
                     <div class="d-flex flex-row">
                        <div class="row-tag-entry">
                           <div class="tags">Voller Name</div>
                           <div class="notes">Tanaka Ikkou</div>
                        </div>
                        <div class="row-tag-entry">
                           <div class="tags">Geschlecht</div>
                           <div class="notes">Männlich</div>
                        </div>
                        <div class="row-tag-entry">
                           <div class="tags">Namensursprung</div>
                           <div class="notes">Berühmter Künstler</div>
                        </div>
                     </div>
                     <hr/>
                  </div>
                  <!-- !!! End of Name Template !!! -->
              </div>
            </div>
          </div>
        </div>
      </div>

   </body>
</html>