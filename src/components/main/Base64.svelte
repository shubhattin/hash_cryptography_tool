<form>
  <div class="grid">
    <label>
      <input type="radio" name="option" value="encode" checked />
      Encode
    </label>
    <label>
      <input type="radio" name="option" value="decode" />
      Deocde
    </label>
  </div>
  <label>
    Enter Text
    <textarea name="text" required></textarea>
  </label>
  <button type="submit">Sumbit</button>
  <label>
    Output Text
    <span class="clear_btn"></span>
    <span class="copy_btn"></span>
    <span class="spinner htmx-indicator" id="base64_spinner"></span>
    <div id="base64_out">
      <textarea readonly></textarea>
    </div>
  </label>
</form>
